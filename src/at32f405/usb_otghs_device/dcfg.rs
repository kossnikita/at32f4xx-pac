#[doc = "Register `DCFG` reader"]
pub type R = crate::R<DCFG_SPEC>;
#[doc = "Register `DCFG` writer"]
pub type W = crate::W<DCFG_SPEC>;
#[doc = "Field `DEVSPD` reader - Device speed"]
pub type DEVSPD_R = crate::FieldReader;
#[doc = "Field `DEVSPD` writer - Device speed"]
pub type DEVSPD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NZSTSOUTHSHK` reader - Non-zero-length status OUT handshake"]
pub type NZSTSOUTHSHK_R = crate::BitReader;
#[doc = "Field `NZSTSOUTHSHK` writer - Non-zero-length status OUT handshake"]
pub type NZSTSOUTHSHK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEVADDR` reader - Device address"]
pub type DEVADDR_R = crate::FieldReader;
#[doc = "Field `DEVADDR` writer - Device address"]
pub type DEVADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PERFRINT` reader - Periodic frame interval"]
pub type PERFRINT_R = crate::FieldReader;
#[doc = "Field `PERFRINT` writer - Periodic frame interval"]
pub type PERFRINT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Device speed"]
    #[inline(always)]
    pub fn devspd(&self) -> DEVSPD_R {
        DEVSPD_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Non-zero-length status OUT handshake"]
    #[inline(always)]
    pub fn nzstsouthshk(&self) -> NZSTSOUTHSHK_R {
        NZSTSOUTHSHK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:10 - Device address"]
    #[inline(always)]
    pub fn devaddr(&self) -> DEVADDR_R {
        DEVADDR_R::new(((self.bits >> 4) & 0x7f) as u8)
    }
    #[doc = "Bits 11:12 - Periodic frame interval"]
    #[inline(always)]
    pub fn perfrint(&self) -> PERFRINT_R {
        PERFRINT_R::new(((self.bits >> 11) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCFG")
            .field("devspd", &self.devspd())
            .field("nzstsouthshk", &self.nzstsouthshk())
            .field("devaddr", &self.devaddr())
            .field("perfrint", &self.perfrint())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Device speed"]
    #[inline(always)]
    pub fn devspd(&mut self) -> DEVSPD_W<'_, DCFG_SPEC> {
        DEVSPD_W::new(self, 0)
    }
    #[doc = "Bit 2 - Non-zero-length status OUT handshake"]
    #[inline(always)]
    pub fn nzstsouthshk(&mut self) -> NZSTSOUTHSHK_W<'_, DCFG_SPEC> {
        NZSTSOUTHSHK_W::new(self, 2)
    }
    #[doc = "Bits 4:10 - Device address"]
    #[inline(always)]
    pub fn devaddr(&mut self) -> DEVADDR_W<'_, DCFG_SPEC> {
        DEVADDR_W::new(self, 4)
    }
    #[doc = "Bits 11:12 - Periodic frame interval"]
    #[inline(always)]
    pub fn perfrint(&mut self) -> PERFRINT_W<'_, DCFG_SPEC> {
        PERFRINT_W::new(self, 11)
    }
}
#[doc = "OTGHS device configuration register (OTGHS_DCFG)\n\nYou can [`read`](crate::Reg::read) this register and get [`dcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCFG_SPEC;
impl crate::RegisterSpec for DCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcfg::R`](R) reader structure"]
impl crate::Readable for DCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcfg::W`](W) writer structure"]
impl crate::Writable for DCFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCFG to value 0x0220_0000"]
impl crate::Resettable for DCFG_SPEC {
    const RESET_VALUE: u32 = 0x0220_0000;
}
