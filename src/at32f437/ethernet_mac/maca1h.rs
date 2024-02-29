#[doc = "Register `MACA1H` reader"]
pub type R = crate::R<MACA1H_SPEC>;
#[doc = "Register `MACA1H` writer"]
pub type W = crate::W<MACA1H_SPEC>;
#[doc = "Field `MA1H` reader - MAC address1 high"]
pub type MA1H_R = crate::FieldReader<u16>;
#[doc = "Field `MA1H` writer - MAC address1 high"]
pub type MA1H_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MBC` reader - Mask byte control"]
pub type MBC_R = crate::FieldReader;
#[doc = "Field `MBC` writer - Mask byte control"]
pub type MBC_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SA` reader - Source address"]
pub type SA_R = crate::BitReader;
#[doc = "Field `SA` writer - Source address"]
pub type SA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AE` reader - Address enable"]
pub type AE_R = crate::BitReader;
#[doc = "Field `AE` writer - Address enable"]
pub type AE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - MAC address1 high"]
    #[inline(always)]
    pub fn ma1h(&self) -> MA1H_R {
        MA1H_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 24:29 - Mask byte control"]
    #[inline(always)]
    pub fn mbc(&self) -> MBC_R {
        MBC_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - Source address"]
    #[inline(always)]
    pub fn sa(&self) -> SA_R {
        SA_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Address enable"]
    #[inline(always)]
    pub fn ae(&self) -> AE_R {
        AE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACA1H")
            .field("ma1h", &format_args!("{}", self.ma1h().bits()))
            .field("mbc", &format_args!("{}", self.mbc().bits()))
            .field("sa", &format_args!("{}", self.sa().bit()))
            .field("ae", &format_args!("{}", self.ae().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<MACA1H_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15 - MAC address1 high"]
    #[inline(always)]
    #[must_use]
    pub fn ma1h(&mut self) -> MA1H_W<MACA1H_SPEC> {
        MA1H_W::new(self, 0)
    }
    #[doc = "Bits 24:29 - Mask byte control"]
    #[inline(always)]
    #[must_use]
    pub fn mbc(&mut self) -> MBC_W<MACA1H_SPEC> {
        MBC_W::new(self, 24)
    }
    #[doc = "Bit 30 - Source address"]
    #[inline(always)]
    #[must_use]
    pub fn sa(&mut self) -> SA_W<MACA1H_SPEC> {
        SA_W::new(self, 30)
    }
    #[doc = "Bit 31 - Address enable"]
    #[inline(always)]
    #[must_use]
    pub fn ae(&mut self) -> AE_W<MACA1H_SPEC> {
        AE_W::new(self, 31)
    }
}
#[doc = "Ethernet MAC address 1 high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca1h::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca1h::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACA1H_SPEC;
impl crate::RegisterSpec for MACA1H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maca1h::R`](R) reader structure"]
impl crate::Readable for MACA1H_SPEC {}
#[doc = "`write(|w| ..)` method takes [`maca1h::W`](W) writer structure"]
impl crate::Writable for MACA1H_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACA1H to value 0xffff"]
impl crate::Resettable for MACA1H_SPEC {
    const RESET_VALUE: u32 = 0xffff;
}
