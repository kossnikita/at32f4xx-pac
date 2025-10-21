#[doc = "Register `INTSTS` reader"]
pub type R = crate::R<INTSTS_SPEC>;
#[doc = "Register `INTSTS` writer"]
pub type W = crate::W<INTSTS_SPEC>;
#[doc = "Field `EPT_NUM` reader - Endpoint number"]
pub type EPT_NUM_R = crate::FieldReader;
#[doc = "Field `EPT_NUM` writer - Endpoint number"]
pub type EPT_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `INOUT` reader - In/Out transaction"]
pub type INOUT_R = crate::BitReader;
#[doc = "Field `INOUT` writer - In/Out transaction"]
pub type INOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSOF` reader - Lost start of frame"]
pub type LSOF_R = crate::BitReader;
#[doc = "Field `LSOF` writer - Lost start of frame"]
pub type LSOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOF` reader - start of frame"]
pub type SOF_R = crate::BitReader;
#[doc = "Field `SOF` writer - start of frame"]
pub type SOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST` reader - Bus reset"]
pub type RST_R = crate::BitReader;
#[doc = "Field `RST` writer - Bus reset"]
pub type RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SP` reader - Bus suspend"]
pub type SP_R = crate::BitReader;
#[doc = "Field `SP` writer - Bus suspend"]
pub type SP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WK` reader - Wakeup"]
pub type WK_R = crate::BitReader;
#[doc = "Field `WK` writer - Wakeup"]
pub type WK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BE` reader - Bus error"]
pub type BE_R = crate::BitReader;
#[doc = "Field `BE` writer - Bus error"]
pub type BE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCFOR` reader - USB core fifo overrun memory"]
pub type UCFOR_R = crate::BitReader;
#[doc = "Field `UCFOR` writer - USB core fifo overrun memory"]
pub type UCFOR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC` reader - transaction completed"]
pub type TC_R = crate::BitReader;
#[doc = "Field `TC` writer - transaction completed"]
pub type TC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Endpoint number"]
    #[inline(always)]
    pub fn ept_num(&self) -> EPT_NUM_R {
        EPT_NUM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - In/Out transaction"]
    #[inline(always)]
    pub fn inout(&self) -> INOUT_R {
        INOUT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Lost start of frame"]
    #[inline(always)]
    pub fn lsof(&self) -> LSOF_R {
        LSOF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - start of frame"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Bus reset"]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Bus suspend"]
    #[inline(always)]
    pub fn sp(&self) -> SP_R {
        SP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Wakeup"]
    #[inline(always)]
    pub fn wk(&self) -> WK_R {
        WK_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Bus error"]
    #[inline(always)]
    pub fn be(&self) -> BE_R {
        BE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - USB core fifo overrun memory"]
    #[inline(always)]
    pub fn ucfor(&self) -> UCFOR_R {
        UCFOR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - transaction completed"]
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTSTS")
            .field("ept_num", &self.ept_num())
            .field("inout", &self.inout())
            .field("lsof", &self.lsof())
            .field("sof", &self.sof())
            .field("rst", &self.rst())
            .field("sp", &self.sp())
            .field("wk", &self.wk())
            .field("be", &self.be())
            .field("ucfor", &self.ucfor())
            .field("tc", &self.tc())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Endpoint number"]
    #[inline(always)]
    pub fn ept_num(&mut self) -> EPT_NUM_W<'_, INTSTS_SPEC> {
        EPT_NUM_W::new(self, 0)
    }
    #[doc = "Bit 4 - In/Out transaction"]
    #[inline(always)]
    pub fn inout(&mut self) -> INOUT_W<'_, INTSTS_SPEC> {
        INOUT_W::new(self, 4)
    }
    #[doc = "Bit 8 - Lost start of frame"]
    #[inline(always)]
    pub fn lsof(&mut self) -> LSOF_W<'_, INTSTS_SPEC> {
        LSOF_W::new(self, 8)
    }
    #[doc = "Bit 9 - start of frame"]
    #[inline(always)]
    pub fn sof(&mut self) -> SOF_W<'_, INTSTS_SPEC> {
        SOF_W::new(self, 9)
    }
    #[doc = "Bit 10 - Bus reset"]
    #[inline(always)]
    pub fn rst(&mut self) -> RST_W<'_, INTSTS_SPEC> {
        RST_W::new(self, 10)
    }
    #[doc = "Bit 11 - Bus suspend"]
    #[inline(always)]
    pub fn sp(&mut self) -> SP_W<'_, INTSTS_SPEC> {
        SP_W::new(self, 11)
    }
    #[doc = "Bit 12 - Wakeup"]
    #[inline(always)]
    pub fn wk(&mut self) -> WK_W<'_, INTSTS_SPEC> {
        WK_W::new(self, 12)
    }
    #[doc = "Bit 13 - Bus error"]
    #[inline(always)]
    pub fn be(&mut self) -> BE_W<'_, INTSTS_SPEC> {
        BE_W::new(self, 13)
    }
    #[doc = "Bit 14 - USB core fifo overrun memory"]
    #[inline(always)]
    pub fn ucfor(&mut self) -> UCFOR_W<'_, INTSTS_SPEC> {
        UCFOR_W::new(self, 14)
    }
    #[doc = "Bit 15 - transaction completed"]
    #[inline(always)]
    pub fn tc(&mut self) -> TC_W<'_, INTSTS_SPEC> {
        TC_W::new(self, 15)
    }
}
#[doc = "interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`intsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTSTS_SPEC;
impl crate::RegisterSpec for INTSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intsts::R`](R) reader structure"]
impl crate::Readable for INTSTS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intsts::W`](W) writer structure"]
impl crate::Writable for INTSTS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTSTS to value 0"]
impl crate::Resettable for INTSTS_SPEC {}
