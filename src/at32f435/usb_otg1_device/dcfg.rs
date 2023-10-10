#[doc = "Register `DCFG` reader"]
pub type R = crate::R<DCFG_SPEC>;
#[doc = "Register `DCFG` writer"]
pub type W = crate::W<DCFG_SPEC>;
#[doc = "Field `DEVSPD` reader - Device speed"]
pub type DEVSPD_R = crate::FieldReader;
#[doc = "Field `DEVSPD` writer - Device speed"]
pub type DEVSPD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `NZSTSOUTHSHK` reader - Non-zero-length status OUT handshake"]
pub type NZSTSOUTHSHK_R = crate::BitReader;
#[doc = "Field `NZSTSOUTHSHK` writer - Non-zero-length status OUT handshake"]
pub type NZSTSOUTHSHK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DEVADDR` reader - Device address"]
pub type DEVADDR_R = crate::FieldReader;
#[doc = "Field `DEVADDR` writer - Device address"]
pub type DEVADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `PERFRINT` reader - Periodic frame interval"]
pub type PERFRINT_R = crate::FieldReader;
#[doc = "Field `PERFRINT` writer - Periodic frame interval"]
pub type PERFRINT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
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
            .field("devspd", &format_args!("{}", self.devspd().bits()))
            .field(
                "nzstsouthshk",
                &format_args!("{}", self.nzstsouthshk().bit()),
            )
            .field("devaddr", &format_args!("{}", self.devaddr().bits()))
            .field("perfrint", &format_args!("{}", self.perfrint().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<DCFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - Device speed"]
    #[inline(always)]
    #[must_use]
    pub fn devspd(&mut self) -> DEVSPD_W<DCFG_SPEC, 0> {
        DEVSPD_W::new(self)
    }
    #[doc = "Bit 2 - Non-zero-length status OUT handshake"]
    #[inline(always)]
    #[must_use]
    pub fn nzstsouthshk(&mut self) -> NZSTSOUTHSHK_W<DCFG_SPEC, 2> {
        NZSTSOUTHSHK_W::new(self)
    }
    #[doc = "Bits 4:10 - Device address"]
    #[inline(always)]
    #[must_use]
    pub fn devaddr(&mut self) -> DEVADDR_W<DCFG_SPEC, 4> {
        DEVADDR_W::new(self)
    }
    #[doc = "Bits 11:12 - Periodic frame interval"]
    #[inline(always)]
    #[must_use]
    pub fn perfrint(&mut self) -> PERFRINT_W<DCFG_SPEC, 11> {
        PERFRINT_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "OTGFS device configuration register (OTGFS_DCFG)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCFG_SPEC;
impl crate::RegisterSpec for DCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcfg::R`](R) reader structure"]
impl crate::Readable for DCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcfg::W`](W) writer structure"]
impl crate::Writable for DCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCFG to value 0x0220_0000"]
impl crate::Resettable for DCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0220_0000;
}
